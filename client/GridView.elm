module GridView exposing (..)

import Svg exposing (..)
import Svg.Attributes exposing (..)
import Array
import Model.Grid as Grid exposing (Point, Slot, Grid)


grid model =
    svg []
        (Grid.map
            slot
            model.grid
        )


slot : Slot -> Svg msg
slot slot =
    let
        ( xp, yp ) =
            toPercentage slot.position
    in
        image [ xlinkHref "/static/img/grid_slot.svg", x xp, y yp, width "10%", height "10%", class "slot" ] []


toPercentage : Point -> ( String, String )
toPercentage point =
    ( (toString <| point.x * 10) ++ "%", (toString <| point.y * 10) ++ "%" )
