module Model exposing (..)

import Model.Grid
import Model.User exposing (User)
import Model.Modules exposing (..)
import Model.Research exposing (..)
import Animation exposing (..)
import Dict
import Model.Util


type alias Model =
    { grid : Model.Grid.Grid
    , user : User
    , authDialogAnimation : Animation.State
    , availableModules : List Module
    , researches : Dict.Dict String Research
    , buildingAt : Maybe Model.Util.Point
    , mainView : MainView
    }


type MainView
    = GridView
    | ResearchView
