module View.MenuBar exposing (..)

import Css exposing (..)
import Css.Namespace exposing (namespace)
import Html exposing (..)
import Html.CssHelpers
import Html.Events exposing (onClick)
import Messages exposing (Msg(SetMainView))
import Model exposing (MainView(GridView), MainView(ResearchView), Model, MainView(ResourcesView))
import View.Queue


view : Model -> Html.Html Messages.Msg
view model =
    div [ helpers.class [ Container ] ]
        [ switchButton model.mainView GridView "Base"
        , switchButton model.mainView ResearchView "Research"
        , switchButton model.mainView ResourcesView "Resources"
        , View.Queue.view model
        ]


switchButton : MainView -> MainView -> String -> Html Msg
switchButton activeView viewType name =
    let
        classes =
            if activeView == viewType then
                helpers.class [ Button, ActiveButton ]
            else
                helpers.class [ Button ]
    in
        button [ onClick (SetMainView viewType), classes ] [ Html.text name ]


type Classes
    = Container
    | Button
    | ActiveButton


rules : Stylesheet
rules =
    let
        barWidth =
            (vw 20)
    in
        (stylesheet << namespace ns)
            [ Css.class Container
                [ width barWidth
                , maxWidth (Css.em 10)
                , minWidth (Css.em 6)
                , displayFlex
                , flexDirection column
                , alignItems center
                ]
            , Css.class Button
                [ width (pct 100)
                , height (Css.em 4)
                , margin zero
                , borderStyle none
                , backgroundColor (rgba 170 170 204 0.4)
                , borderLeft3 (px 5) solid (rgba 170 170 204 1)
                ]
            , Css.class ActiveButton
                [ backgroundColor (rgba 170 170 204 1)
                , borderLeftStyle none
                ]
            ]


ns : String
ns =
    "menubar"


helpers : Html.CssHelpers.Namespace String class id msg
helpers =
    Html.CssHelpers.withNamespace ns
