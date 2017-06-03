module Model.Resources exposing (..)


type alias Resources =
    List Resource


type alias Resource =
    { amount : Int
    , id : String
    , maxAmount : Int
    , name : String
    }
