module Model exposing (..)

import Model.Grid exposing (..)
import Model.User exposing (User)
import Model.Modules exposing (..)
import Model.Research exposing (..)
import Animation exposing (..)
import Dict


type alias Model =
    { grid : Grid
    , user : User
    , authDialogAnimation : Animation.State
    , authView : AuthView
    , availableModules : List Module
    , researches : Dict.Dict String Research
    }


type AuthView
    = Register
    | Login
