module Model exposing (..)

import Model.Grid exposing (..)
import Model.User exposing (User)
import Model.Modules exposing (..)
import Animation exposing (..)


type alias Model =
    { grid : Grid
    , user : User
    , authDialogAnimation : Animation.State
    , authView : AuthView
    , availableModules : List Module
    }


type AuthView
    = Register
    | Login
