local cam1 = Camera.create(Transform(0, 0))
local cam2 = Camera.create(Transform(-0.5, -0.5))

function update(dt: number)
	if Input.isPressed(Input.KeyCode.Q) then
		cam1:setPrimary()
	elseif Input.isPressed(Input.KeyCode.E) then
		cam2:setPrimary()
	end
end
