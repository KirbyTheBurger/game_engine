Camera.new(Transform(0, 0)):setPrimary()

local texture = Texture.new("brownie.png")

local sprite1 = Sprite.new(0, 0, texture)
local sprite2 = Sprite.new(0.5, 0, texture)

sprite1.scale = Transform(2, 1)
sprite1.zindex = 0
sprite2.zindex = 0.5
sprite2.rotation = 30

const MOV_SPEED = 2;
const ROT_SPEED = 5;

local sprite3 = Sprite.new(0, 0, texture)
sprite3.zindex = 1
sprite3.scale = Transform(0.5, 0.5)

function update(dt: number)
	if Input.isPressed(Input.KeyCode.W) then
		sprite3.position += Transform(0, dt * MOV_SPEED)
	elseif Input.isPressed(Input.KeyCode.S) then
		sprite3.position -= Transform(0, dt * MOV_SPEED)
	elseif Input.isPressed(Input.KeyCode.A) then
		sprite3.position -= Transform(dt * MOV_SPEED, 0)
	elseif Input.isPressed(Input.KeyCode.D) then
		sprite3.position += Transform(dt * MOV_SPEED, 0)
	end

	if Input.isPressed(Input.KeyCode.Q) then
		sprite3.rotation += dt * ROT_SPEED
	elseif Input.isPressed(Input.KeyCode.E) then
		sprite3.rotation -= dt * ROT_SPEED
	end
end
