module Messages exposing (..)

import Animation
import Model.User exposing (..)
import Model
import Http
import Model.Util exposing (..)
import Model.Research


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
    | ReceiveResearches (Result Http.Error Model.Research.Researches)
