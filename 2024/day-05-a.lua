-- Read rules
local rules = {}
local inputRule = io.read()
while inputRule and #inputRule ~= 0 do
	local required, number = inputRule:gmatch("(%d+)|(%d+)")()
	if not rules[number] then
		rules[number] = {}
	end
	table.insert(rules[number], required)
	inputRule = io.read()
end

-- Read lines
local lines = {}
local inputLine = io.read()
while inputLine and #inputLine ~= 0 do
	local line = {}
	for currentPage in inputLine:gmatch("(%d+)") do
		table.insert(line, currentPage)
	end
	table.insert(lines, line)
	inputLine = io.read()
end

local function lineIsCorrect(line)
	for currentIndex, current in pairs(line) do
		if not rules[current] then
			goto continue
		end
		for _, ruleToAply in pairs(rules[current]) do
			local isCorrect, aplyRule = false, false
			for numIndex, numToCheck in pairs(line) do
				if ruleToAply == numToCheck then
					if numIndex <= currentIndex then
						isCorrect = true
					end
					aplyRule = true
				end
			end
			if not isCorrect and aplyRule then
				return false
			end
		end
		::continue::
	end
	return true
end

local sum = 0
for _, line in pairs(lines) do
	if lineIsCorrect(line) then
		local i = math.ceil(#line / 2)
		sum = sum + tonumber(line[i])
	end
end
print(sum)
