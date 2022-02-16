# rusteroids

a clone (enhancement?) of the classic asteroids game

# Process

- shot speed increase with ship velocity?
---------

---------
- sounds
- shooting
    - //delay
    - //max amount of bullets on screen
    - //bullet lifetime
    - //bullets dissapear when they hit an asteroid
    - //collision detections
- asteroids split when hit
- multiple asteroids
    - problem when collision checking with moved values --> solved with .iter_mut() 
    https://pr0.uk/rust/programming/loop/borrow-checker/2019/09/02/rust-inner-loop.html
- collision detection https://happycoding.io/tutorials/processing/collision-detection
    eerst maken we een simpele circle - circle CD systeem, later kunnen we met line instersection werken https://gamedev.stackexchange.com/questions/89929/collision-detection-between-triangle-and-polygon
    collision detection moet eigenlijk bij elke individuele update gebeuren (traits gebruiken), nu gebeurt het nog 1 keer
- asteroids (with intertia)
- ship physics (inertia and thrusters)
- cooler spaceship
- triangle on the screen
    - needs center and angle
        - calculate endpoint given startpoint and angle:
        Starting point you know (x1, x2), end point is (x1 + l * cos(ang), y1 + l * sin(ang)) where l is the length and ang is the angle.
        - 2PI is een volledige cirkel!
- macroquad window

## Refactor

- no magic numbers
- asteroid split-when-hit functionality 
    -> one loop or two?
    -> magic number 20 (how many times does it split?)
- cleanup_asteroids (unnecessary loop?)
- input met match pattern

------------
- 1 function for bounds-checking shared by all collidable objects
------------
- put main code in a game object that handles game state, with an update and a draw function
- input code in seperate module
- refactor update loop into subfunctions    
- gameloop ownership issues
    gebruik iter_mut() voor een iterator waar je mutable elementen moet hebben
- collision detection //met traits met faction flag (zodat als er collision is, je kan kijken naar friendly fire)
    todo: 
        - collision detection 'timing' 
            (test collision of ship against all asteroids, vice verse not necessary,
             test collision of all bullets against all asteroids)
        - program consequence of collision into traitobject + implementation
- aparte bestandjes

## MVP

- //ship (triangle)
- //that can move (thrust from back of ship, angular movement)
- //and can shoot ( little circles)
- //at asteroids (irregular polygons)
- //that divide when shot (smaller irregular polygons)
- also, there are spaceships that fire back
- //when you are hit, you lose a life (collision detection)
- //scoring

## Enhancement

- collision detection with line-line intersection 'pixel perfect' https://gamedev.stackexchange.com/questions/89929/collision-detection-between-triangle-and-polygon
- ship hit-delay grace period
    - otherwise it wil keep getting hit 
    