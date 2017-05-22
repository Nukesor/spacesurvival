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
