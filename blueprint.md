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

This should attract the weaker mass object, but in reality, planets dont just always crash into the star, they orbit! We can apply the escape velocity formula.

- Use this formula: vescape = sqrt((2*G*M)/r)
- G = Gravitational Constant
- M = Mass of the body to be escaped from
- r = Distance from the center of the mass (or the heavier object)

If the object's velocity is greater than or equal to vescape, it will get kicked out of its orbit. If it's lower, it either orbits or falls in.

Now we need to make sure that the planet stays in its orbit. Using the orbital velocity that is.

- Use this formula: vorbit = sqrt((G*M)/r)
- G = Gravitational Constant
- M = Mass of the body to be escaped from
- r = Distance from the center of the mass (or the heavier object)

if velocity is less than vorbit, the object will spiral inward and crash.
if velocity is equal to vorbit, the object will orbit.
if velocity is between vorbit and vescape, it will follow an eliptical orbit.
if velocity is greater vescape, it escapes the system.

A little bit of bonus:
Simulating Motion with Newton's Laws

If you want to simulate movement step by step, use acceleration:

a = F/m
a = acceleration, F = Force, m = mass

- Calculate acceleration due to gravity
- Update velocity and position over time (Euler or Verlet integration)

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
