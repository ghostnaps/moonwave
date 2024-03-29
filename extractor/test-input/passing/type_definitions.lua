--[=[
	Class description

	@class TypeDefs
]=]

--[=[
	A description of the type

	@within TypeDefs
]=]
export type Foo = {
	-- A description for the field
	field: number?,

	-- A description for this field too.
	-- With a second line.
	-- And a third
	another: string?,
}

--[=[
	A description of the type

	@within TypeDefs
]=]
export type MultilineFieldComments = {
	--[[
		Now with a multiline comment

		For cases where there's a lot to say about a field
	]]
	field: boolean,
}
