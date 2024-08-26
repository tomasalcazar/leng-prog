module Shape where

data Point = Point { x :: Double, y :: Double } deriving (Eq, Show)

data Circle    = Circle Point Double deriving (Eq, Show)
data Rectangle = Rectangle Point Point deriving (Eq, Show)

point :: (Double, Double) -> Point
point (x, y) = Point { x = x, y = y }

origin :: Point
origin = Point { x = 0.0, y = 0.0 }

rectangle :: (Double, Double) -> Rectangle
rectangle (x, y) = Rectangle origin (Point { x = x, y = y })

base :: Rectangle -> Double
base (Rectangle (Point x0 _) (Point x1 _)) = abs (x1 - x0)

height :: Rectangle -> Double
height (Rectangle (Point _ y0) (Point _ y1)) = abs (y1 - y0)

circle :: Double -> Circle
circle r = Circle origin r

class Shift a where
   shift :: a -> (Double, Double) -> a

instance Shift Point where
   shift (Point x y) (dx, dy) = Point (x + dx) (y + dy)

instance Shift Rectangle where
   shift (Rectangle p1 p2) (dx, dy) = Rectangle (shift p1 (dx, dy)) (shift p2 (dx, dy))

instance Shift Circle where
   shift (Circle center r) (dx, dy) = Circle (shift center (dx, dy)) r

class Surface a where
   surface :: a -> Double

instance Surface Rectangle where
   surface rect = base rect * height rect

instance Surface Circle where
   surface (Circle _ r) = pi * r * r
