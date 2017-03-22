module Model exposing (..)

import Components.Modal
import Array exposing (Array)
import Model.Grid exposing (..)


type alias Model =
    { modals : List Components.Modal.Model, grid : Grid }
