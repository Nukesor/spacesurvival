module View.Grid exposing (..)

import Css exposing (..)
import Css.Namespace exposing (namespace)
import Html exposing (..)
import Html.Attributes
import Html.CssHelpers
import Messages exposing (..)
import Model exposing (..)
import Model.Grid as Grid exposing (Grid, Slot)
import Model.Util exposing (..)
import Svg exposing (..)
import Svg.Attributes
import Svg.Events exposing (..)
import View.BuildDialog


view : Model -> Html.Html Msg
view model =
    div [ Html.Attributes.class "grid-container", helpers.class [ CenterContainer ] ]
        [ div [ helpers.class [ Container ] ]
            [ svg [ Svg.Attributes.width "100%", Svg.Attributes.height "100%" ]
                (List.concat
                    (Grid.map
                        slot
                        model.grid
                    )
                )
            , View.BuildDialog.view model
            ]
        ]


slot : Slot -> List (Svg Msg)
slot slot =
    let
        ( xp, yp ) =
            toPercentage slot.position
    in
        case slot.entry of
            Just mod ->
                [ image
                    [ Svg.Attributes.xlinkHref "/static/img/module.svg"
                    , Svg.Attributes.x xp
                    , Svg.Attributes.y yp
                    , Svg.Attributes.width "10%"
                    , Svg.Attributes.height "10%"
                    ]
                    []
                ]

            Nothing ->
                [ image
                    [ Svg.Attributes.xlinkHref "/static/img/grid_slot.svg"
                    , Svg.Attributes.x xp
                    , Svg.Attributes.y yp
                    , Svg.Attributes.width "10%"
                    , Svg.Attributes.height "10%"
                    , Svg.Attributes.class "slot"
                    , onClick (ShowBuildDialog (Just slot.position))
                    ]
                    []
                ]


toPercentage : Point -> ( String, String )
toPercentage point =
    ( (toString <| point.x * 10) ++ "%", (toString <| point.y * 10) ++ "%" )


type Classes
    = Container
    | CenterContainer


rules : Stylesheet
rules =
    let
        gridSize =
            80
    in
        (stylesheet << namespace ns)
            [ Css.class Container
                [ maxWidth (vw gridSize)
                , maxHeight (vw gridSize)
                , width (vh gridSize)
                , height (vh gridSize)
                , displayFlex
                , justifyContent center
                , alignItems center
                , padding (pct 3)
                ]
            , Css.class CenterContainer
                [ displayFlex
                , justifyContent center
                , alignItems center
                , width (pct 100)
                , height (pct 100)
                ]
            ]


ns : String
ns =
    "grid"


helpers : Html.CssHelpers.Namespace String class id msg
helpers =
    Html.CssHelpers.withNamespace ns
