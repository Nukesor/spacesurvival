module Model exposing (..)

import Animation exposing (..)
import Dict
import Model.Grid
import Model.Modules exposing (..)
import Model.Queue exposing (Queue)
import Model.Research exposing (..)
import Model.User exposing (User)
import Model.Util


type alias Model =
    { grid : Model.Grid.Grid
    , user : User
    , authDialogAnimation : Animation.State
    , availableModules : Modules
    , researches : Dict.Dict String Research
    , buildingAt : Maybe Model.Util.Point
    , mainView : MainView
    , queue : Queue
    }


type MainView
    = GridView
    | ResearchView
