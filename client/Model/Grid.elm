module Model.Grid exposing (..)

import Array
import Extra.Maybe exposing (isJust)
import List
import Model.Modules exposing (..)
import Model.Util exposing (..)


type alias Grid =
    Array.Array (Array.Array Slot)


type alias Slot =
    { position : Point
    , entry : SlotEntry
    }


modules : Grid -> List Module
modules grid =
    grid
        |> applyFunctor List.filterMap .entry


applyFunctor : (fn -> List Slot -> List b) -> fn -> Grid -> List b
applyFunctor functor fn grid =
    grid
        |> Array.map (\ys -> functor fn (Array.toList ys))
        |> Array.toList
        |> List.concat


map : (Slot -> a) -> Grid -> List a
map fn grid =
    applyFunctor List.map fn grid


filter : (Slot -> Bool) -> Grid -> List Slot
filter fn grid =
    applyFunctor List.filter fn grid


atPosition : Point -> Grid -> Maybe Slot
atPosition point grid =
    Array.get point.x grid
        |> Maybe.andThen (Array.get point.y)


setAtPosition : Point -> Module -> Grid -> Grid
setAtPosition point mod grid =
    let
        updatedSlot =
            atPosition point grid
                |> Maybe.map (\slot -> { slot | entry = Just mod })

        row =
            Array.get point.x grid
    in
        case ( row, updatedSlot ) of
            ( Just row, Just slot ) ->
                Array.set point.x (Array.set point.y slot row) grid

            _ ->
                grid


empty : Grid
empty =
    Array.initialize 10
        (\x ->
            Array.initialize 10
                (\y ->
                    { position = Point x y
                    , entry = Nothing
                    }
                )
        )
