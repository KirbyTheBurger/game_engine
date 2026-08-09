declare extern type Texture with

end

declare Texture: {
    new: (path: string) -> Texture
}

declare extern type Sprite with
    position: Transform
    rotation: number
    scale: Transform
    zindex: number
    texture: Texture
end

declare Sprite: {
    new: (x: number, y: number, texture: Texture) -> Sprite
}
