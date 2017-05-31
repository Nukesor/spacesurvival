module View.Research exposing (..)

import Css exposing (..)
import Css.Namespace exposing (namespace)
import Dict
import Html exposing (..)
import Html.CssHelpers
import Messages
import Model exposing (Model)
import Model.Research exposing (Research)


view : Model -> Html Messages.Msg
view model =
    div []
        [ ul []
            (List.map
                researchItem
                (Dict.values model.researches)
            )
        ]


researchItem : Research -> Html Messages.Msg
researchItem research =
    li [] [ Html.text research.name ]


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
