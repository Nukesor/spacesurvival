module Model.User exposing (..)


type User
    = LoggingIn LoginData
    | Registering RegisterData
    | LoggedIn LoggedInData


type alias LoginData =
    { identifier : String, password : String }


type alias RegisterData =
    { email : String, password : String, nickname : String }


type alias LoggedInData =
    { token : String, id : String, podId : String }
