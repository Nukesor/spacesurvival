module View.Layout exposing (..)

import Css exposing (..)
import Css.Namespace exposing (namespace)
import Html exposing (..)
import Html.CssHelpers
import Messages exposing (Msg)
import Model exposing (..)
import View.Grid
import View.MenuBar
import View.Research


view : Model.Model -> Html Msg
view model =
    div [ helpers.class [ Container ] ]
        [ div [ helpers.class [ MainViewContainer ] ] [ mainView model.mainView model ]
        , View.MenuBar.view model
        ]


mainView : MainView -> (Model -> Html Msg)
mainView viewType =
    case viewType of
        GridView ->
            View.Grid.view

        ResearchView ->
            View.Research.view


type Classes
    = Container
    | MainViewContainer


rules : Stylesheet
rules =
    (stylesheet << namespace ns)
        [ Css.class Container
            [ width (vw 100)
            , height (vh 100)
            , displayFlex
            , flexFlow2 row noWrap
            ]
        , Css.class MainViewContainer
            [ width (vw 90)
            ]
        ]


ns : String
ns =
    "layout"


helpers : Html.CssHelpers.Namespace String class id msg
helpers =
    Html.CssHelpers.withNamespace ns
