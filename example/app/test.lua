test = {}
test.__index = test

function test:create(message)
    local instance = {}
    setmetatable(instance, test)

    instance.message = message

    return instance
end

obj:me_print()
-- test
obj:me_print()

function test:me_print()
    print(self.message)
end

obj = test:create("I'm test")
obj:me_print()
