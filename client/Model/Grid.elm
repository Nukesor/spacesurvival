module Model.Grid exposing (..)

import Array
import List


type alias Grid =
    Array.Array (Array.Array Slot)


type alias Slot =
    { position : Point
    }


type alias Point =
    { x : Int
    , y : Int
    }


map : (Slot -> a) -> Grid -> List a
map fn grid =
    grid
        |> Array.map (\ys -> Array.toList (Array.map fn ys))
        |> Array.toList
        |> List.concat
