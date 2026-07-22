const SPEED = 3;

local cam = Camera.create(Transform(0, 0))
cam:setPrimary()

function update(dt: number)
	if Input.isPressed(Input.KeyCode.W) then
		cam.pos += Transform(0, SPEED * dt)
	elseif Input.isPressed(Input.KeyCode.S) then
		cam.pos -= Transform(0, SPEED * dt)
	elseif Input.isPressed(Input.KeyCode.A) then
		cam.pos -= Transform(SPEED * dt, 0)
	elseif Input.isPressed(Input.KeyCode.D) then
		cam.pos += Transform(SPEED * dt, 0)
	end
end
