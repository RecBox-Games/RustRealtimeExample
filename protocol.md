### Legend / Special Character
* `<>` encloses a variable
* `{}` encloses an optional
* `|`  separates the options within an optional
* `[]` encloses an expression


## Controlpads to Game
* `state-request`
  * Sent by controlpad when it doesn't know what it's current state is supposed 
      to be. Game must respond with a state message.

* `join`
  * Sent when a new player presses the 'Join' button. The game handles it by 
    registering a new player.

* `move`
  * Sent when a player taps on their navigation circle. The game handles it by 
    setting the player's target location

* `hit`
  * sent when a player taps on an object within their navigation circle


## Game to Controlpads
* `state:{joining|[playing]}`
  * Sent in response to a state request. Causes the controller to be updated 
      to reflect the given state.
  * e.g. `state:playing:red:1454,321:1228,404:tree,120,445;rock,992,1001`

* `[playing]`: `playing:color:[location-player]:[location-target]:[field]`

* `[field]` : `[object-1];[object-2];...;[object-N]`

* `[object]` : `{tree|rock},[location]`

* `[location]`: `{<x>,<y>}`
