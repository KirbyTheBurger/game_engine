# Introduction
This is a minimal 2d game engine, made in Rust. It features a luau api for scripting with camera movement, sprites and input detection. It uses WebGPU for rendering, but only uses the Vulkan functionality so games are not embeddable in the browser.
For an example of the game engine's functionality, see this
[simple platformer](https://github.com/user-attachments/assets/a89f83f3-c54f-4ee7-97a9-c93443ee0a90).

# Documentation
Below is the documentation. Do note that the game engine is very minimal and because of that impractical to genuinely use for making games.

## Sprites
To create a new sprite, use Sprite.new(), which takes a texture instance as an argument:
```luau
local texture = Texture.new("./brownie.png")
local sprite = Sprite.new(texture)
```

Sprites have various properties to edit:
```luau
local texture1 = Texture.new("./foo.png")
local texture2 = Texture.new("./bar.png")
local sprite = Sprite.new(texture1)

sprite.position = Transform(5, 2)
sprite.rotation += 30
sprite.scale = Transform(1.5, 2.3)
sprite.zindex = 0.5
sprite.texture = texture2
```

## Cameras
```luau
local cam1 = Camera.new()
local cam2 = Camera.new()

cam1:setPos(2, 2)
cam1:setPos(-2, -2)

-- setPrimary switches the camera being used by the engine
cam1:setPrimary()
cam2:setPrimary()
```

## Input
```luau
if Input.isPressed(Input.KeyCode.W) then
  print("W is pressed")
end
```

## Runtime api
```luau
-- Fires every frame
Runtime.onUpdate(function(dt: number)
  print("current delta time is:", dt)
end)
```
