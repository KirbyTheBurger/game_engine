declare Input: {
    isPressed: (string) -> boolean,

    KeyCode: {
        A: string, B: string, C: string, D: string, E: string, F: string, G: string,
        H: string, I: string, J: string, K: string, L: string, M: string, N: string,
        O: string, P: string, Q: string, R: string, S: string, T: string, U: string,
        V: string, W: string, X: string, Y: string, Z: string,
    }
}

declare Camera: {
    new: (number, number) -> ()
}

declare extern type Transform with
    x: number
    y: number
end

declare function Transform(x: number, y: number): Transform

declare extern type Camera with
    pos: Transform
    function setPrimary(self): ()
    function setPos(self, x: number, y: number): ()
end

declare Camera: {
    create: (pos: Transform) -> Camera
}