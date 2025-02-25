### Objects

First, we are going to define some classes for our space objects. It will be simple, we only need to define a trait and then make a struct for each object.

Trait for pre-defining some functions.
Struct for defining the properties of each object.
Struct is also used to add those functions to the objects. Limited to whatever the trait defines.

### Gravity

If an object has mass, it should either attract other objects, or get attracted (this some flowey kill or be killed shit lol)

- Use Newton's law: F = G * (m1*m2) / (r^2)
- F = Gravitational Force
- G = Gravitational Constant
- m1 = Mass of object 1
- m2 = Mass of object 2
- r = Distance between the centers of the two objects

### Collision

When two objects collide, there are 3 possible things that can happen (at least in this project):

- Bounce off -> Elastic collisions (like asteroids).
- Merge -> If one is much larger (like a spaceship crashing into a planet)
- Break apart -> Asteroids splits into pieces

For collision, we can simply take this math formula:
d = (x1-x2)^2+(y1-y2)^2

- d = distance
- (x1, y1) = center of circle 1
- (x2, y2) = center of circle 2

Note: This actually doesnt take account for square root, so you're gonna have to calculate all that square root inside your head... which isn't a lot :)

We would have another problem though, which is deciding what we should do once both of those objects collide into each other. Bounce off, Merge, Break apart, we can implement some logic into it but it will take more code.

### Movement

Objects should move based on their velocity and acceleration.

### Rotation and Orientation

Objects can spin due to forces
