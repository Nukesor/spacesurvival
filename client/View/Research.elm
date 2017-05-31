module View.Research exposing (..)

import Css exposing (..)
import Css.Namespace exposing (namespace)
import Html exposing (..)
import Html.CssHelpers
import Messages
import Model exposing (Model)


view : Model -> Html Messages.Msg
view model =
    div [] []


rules : Stylesheet
rules =
    (stylesheet << namespace ns)
        []


ns : String
ns =
    "research"


helpers : Html.CssHelpers.Namespace String class id msg
helpers =
    Html.CssHelpers.withNamespace ns
