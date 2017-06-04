module Model.Grid exposing (..)

import Array
import List
import Model.Modules exposing (..)
import Model.Util exposing (..)


type alias Grid =
    Array.Array (Array.Array Slot)


type alias Slot =
    { position : Point
    , entry : SlotEntry
    }


map : (Slot -> a) -> Grid -> List a
map fn grid =
    grid
        |> Array.map (\ys -> Array.toList (Array.map fn ys))
        |> Array.toList
        |> List.concat


atPosition : Int -> Int -> Grid -> Maybe Slot
atPosition x y grid =
    Array.get x grid
        |> Maybe.andThen (Array.get y)


setAtPosition : Int -> Int -> Module -> Grid -> Grid
setAtPosition x y mod grid =
    let
        updatedSlot =
            atPosition x y grid
                |> Maybe.map (\slot -> { slot | entry = Just mod })

        row =
            Array.get x grid
    in
        case ( row, updatedSlot ) of
            ( Just row, Just slot ) ->
                Array.set x (Array.set y slot row) grid

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
