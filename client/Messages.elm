module Messages exposing (..)

import Animation
import Model.User exposing (..)
import Model
import Http
import Model.Util exposing (..)


type Msg
    = AnimateModal Animation.Msg
    | Login
    | Register
    | ChangeAuthView Model.AuthView
    | UpdateUser User
    | Registered (Result Http.Error LoginData)
    | LoggedIn (Result Http.Error LoggedInData)
    | ReadLocalToken LoggedInData
    | ShowBuildDialog (Maybe Point)
