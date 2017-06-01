module View.Research exposing (..)

import Css exposing (..)
import Css.Namespace exposing (namespace)
import Dict
import Html exposing (..)
import Html.CssHelpers
import Html.Events exposing (onClick)
import Messages exposing (Msg(StartResearching))
import Model exposing (Model)
import Model.Research exposing (Research, availableForQueueing)


view : Model -> Html Messages.Msg
view model =
    div []
        [ ul []
            (model.researches
                |> Dict.filter (availableForQueueing model.researches)
                |> Dict.map researchItem
                |> Dict.values
            )
        ]


researchItem : String -> Research -> Html Messages.Msg
researchItem key research =
    li [] [ Html.text research.name, button [ onClick (StartResearching key) ] [ Html.text "Research!" ] ]


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
