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
    , selectedForBuilding : Bool
    }


map : (Slot -> a) -> Grid -> List a
map fn grid =
    grid
        |> Array.map (\ys -> Array.toList (Array.map fn ys))
        |> Array.toList
        |> List.concat
