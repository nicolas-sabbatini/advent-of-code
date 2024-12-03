---@type integer
local acc = 0
---@type string
local input = io.read()
while input and #input ~= 0 do
	for first, second in input:gmatch("mul%((%d+),(%d+)%)") do
		acc = acc + tonumber(first) * tonumber(second)
	end
	input = io.read()
end
print(acc)
