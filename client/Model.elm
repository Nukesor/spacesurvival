module Model exposing (..)

import Animation exposing (..)
import Dict
import Model.Grid
import Model.Modules exposing (..)
import Model.Queue exposing (Queue)
import Model.Research exposing (..)
import Model.Resources exposing (Resources)
import Model.User exposing (User)
import Model.Util
import Time.DateTime exposing (DateTime)
import Time


type alias Model =
    { pod : Model.Grid.Grid
    , base : Model.Grid.Grid
    , user : User
    , authDialogAnimation : Animation.State
    , availableModules : AvailableModules
    , researches : Dict.Dict String Research
    , buildingAt : Maybe Model.Util.Point
    , mainView : MainView
    , queue : Queue
    , resources : Resources
    , currentDate : DateTime
    , lastTick : Maybe Time.Time
    , timeZoneOffset : Float
    }


type MainView
    = GridView SelectedGrid
    | ResearchView
    | ResourcesView


type SelectedGrid
    = Pod
    | Base
