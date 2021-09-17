import fs from "fs-extra"
import path from "path"
import { Config, FoldersEnabled } from "./prepareProject"

export default function getDocusaurusConfig({
  codePaths,
  enablePlugins,
  config,
}: {
  codePaths: string[]
  enablePlugins: FoldersEnabled
  config: Config
}) {
  const gitRepoUrl = config.gitRepoUrl

  const validCodePaths = codePaths
    .map((codePath) => path.join(process.cwd(), codePath))
    .filter((codePath) => fs.existsSync(codePath))

  return {
    title: "You need to configure your title",
    onBrokenLinks: "throw",
    onBrokenMarkdownLinks: "warn",
    favicon: "img/favicon.ico",
    baseUrl: "/",
    url: "localhost",

    ...config.docusaurus,

    themeConfig: {
      hideableSidebar: config.navbar?.hideableSidebar ?? true,

      navbar: {
        ...config.navbar,

        items: [
          ...(enablePlugins.docs
            ? [
                {
                  type: "doc",
                  docId: "intro",
                  position: "left",
                  label: "Docs",
                },
              ]
            : []),
          ...(enablePlugins.blog
            ? [{ to: "/blog", label: "Blog", position: "left" }]
            : []),
          { to: "/api/", label: "API", position: "left" },
          ...(gitRepoUrl
            ? [
                {
                  href: gitRepoUrl,
                  label: "GitHub",
                  position: "right",
                },
              ]
            : []),
          ...(config?.navbar?.items || []),
        ],
      },
      footer: {
        style: "dark",
        copyright: `Copyright © ${new Date().getFullYear()} ${
          config.docusaurus?.organizationName
        }. Built with Moonwave and Docusaurus.`,
        ...config.footer,
      },
    },
    plugins: [
      [
        "docusaurus-plugin-moonwave",
        {
          code: validCodePaths,
          sourceUrl: gitRepoUrl + `/blob/${config.gitSourceBranch ?? "master"}`,
        },
      ],
      "docusaurus-lunr-search",
    ],
    presets: [
      [
        "@docusaurus/preset-classic",
        {
          docs: enablePlugins.docs
            ? {
                // Please change this to your repo.
                editUrl: gitRepoUrl
                  ? `${gitRepoUrl}/edit/${config.gitSourceBranch}/docs/`
                  : undefined, // Omitting this variable entirely will disable edit links
                sidebarCollapsible: true,
              }
            : false,
          blog: enablePlugins.blog
            ? {
                showReadingTime: true,
                // Please change this to your repo.
                editUrl: gitRepoUrl
                  ? `${gitRepoUrl}/edit/${config.gitSourceBranch}/blog/`
                  : undefined, // Omitting this variable entirely will disable edit links
              }
            : false,
          pages: {
            path: "pages",

            //exclude any file starting with an underscore
            exclude: ["_*.*"],
          },
        },
      ],
    ],
  }
}
