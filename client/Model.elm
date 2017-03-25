module Model exposing (..)

import Array exposing (Array)
import Model.Grid exposing (..)
import Model.User exposing (User)
import Animation exposing (..)


type alias Model =
    { grid : Grid, user : User, authDialogAnimation : Animation.State, authView : AuthView }


type AuthView
    = Register
    | Login
