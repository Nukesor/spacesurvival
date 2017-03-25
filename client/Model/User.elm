module Model.User exposing (..)


type alias User =
    { token : Maybe String
    , nickname : String
    , email : String
    , password : String
    }
