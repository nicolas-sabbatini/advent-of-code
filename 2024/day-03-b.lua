---@type boolean
local enableMul = true

local function digest(input)
	---@type integer
	local acc = 0
	while input and #input ~= 0 do
		local match = input:match("^mul%(%d+,%d+%)") or input:match("^do%(%)") or input:match("^don't%(%)")
		if match == "do()" then
			enableMul = true
		elseif match == "don't()" then
			enableMul = false
		elseif match and enableMul then
			for first, second in match:gmatch("mul%((%d+),(%d+)%)") do
				acc = acc + tonumber(first) * tonumber(second)
			end
		end
		input = input:sub(2)
	end
	return acc
end

---@type integer
local res = 0
---@type string
local input = io.read()
while input and #input ~= 0 do
	res = res + digest(input)
	input = io.read()
end
print(res)
