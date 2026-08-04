Camera.new(Transform(0, 0)):setPrimary()

local texture = Texture.new("brownie.png")

local player = Sprite.new(0, 0, texture)
player.scale = Transform(0.5, 0.5)
player.zindex = 1

local GRAVITY = -9.8
local JUMP_FORCE = 3
local MOVE_SPEED = 1.5
local GROUND_Y = -0.75

local velocityY = 0
local isGrounded = true
local jumpKeyWasDown = false

function update(dt: number)
    if Input.isPressed(Input.KeyCode.A) then
        player.position -= Transform(MOVE_SPEED * dt, 0)
    elseif Input.isPressed(Input.KeyCode.D) then
        player.position += Transform(MOVE_SPEED * dt, 0)
    end

    local jumpKeyDown = Input.isPressed(Input.KeyCode.W)
    if jumpKeyDown and not jumpKeyWasDown and isGrounded then
        velocityY = JUMP_FORCE
        isGrounded = false
    end
    jumpKeyWasDown = jumpKeyDown

    velocityY += GRAVITY * dt

    player.position += Transform(0, velocityY * dt)

    if player.position.y <= GROUND_Y then
        player.position = Transform(player.position.x, GROUND_Y)
        velocityY = 0
        isGrounded = true
    end
end
