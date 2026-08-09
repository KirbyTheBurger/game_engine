declare extern type Camera with
    position: Transform
    function setPrimary(self): ()
    function setPos(self, x: number, y: number): ()
    function hello(self): ()
end

declare Camera: {
    new: () -> Camera
}
