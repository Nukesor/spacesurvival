module Messages exposing (..)

import Animation
import Model.User exposing (User)
import Model
import Http
import Dict exposing (Dict)


type Msg
    = AnimateModal Animation.Msg
    | Login
    | Register
    | ChangeAuthView Model.AuthView
    | UpdateUser User
    | Registered (Result Http.Error User)
