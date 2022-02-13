# rusteroids

a clone (enhancement?) of the classic asteroids game

# Process

- shooting
    - delay
    - bullet lifetime
    - collision detections
---------
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

- input met match pattern
- collision detection met traits met faction flag (zodat als er collision is, je kan kijken naar friendly fire)
- aparte bestandjes

## MVP

- ship (triangle)
- that can move (thrust from back of ship, angular movement)
- and can shoot ( little circles)
- at asteroids (irregular polygons)
- that divide when shot (smaller irregular polygons)
- also, there are spaceships that fire back
- when you are hit, you lose a life (collision detection)
- scoring

## Enhancement

- collision detection met line-line intersection 'pixel perfect' https://gamedev.stackexchange.com/questions/89929/collision-detection-between-triangle-and-polygon