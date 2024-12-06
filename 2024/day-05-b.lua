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

local function calculateErrorIndex(line)
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
				return currentIndex
			end
		end
		::continue::
	end
	return nil
end

local function fixLine(line, errorIndex)
	local removes = {}
	while errorIndex do
		table.insert(removes, table.remove(line, errorIndex))
		errorIndex = calculateErrorIndex(line)
	end
	for _, v in pairs(removes) do
		table.insert(line, 1, v)
		local c = 1
		while calculateErrorIndex(line) do
			local h = line[c]
			line[c] = line[c + 1]
			line[c + 1] = h
			c = c + 1
		end
	end
end

local sum = 0
for _, line in pairs(lines) do
	local errorIndex = calculateErrorIndex(line)
	if errorIndex then
		fixLine(line, errorIndex)
		local i = math.ceil(#line / 2)
		sum = sum + tonumber(line[i])
	end
end
print(sum)
