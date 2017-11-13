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
    case model.mainView of
        GridView focused ->
            let
                contents =
                    case focused of
                        Pod ->
                            [ button [ onClick <| SetMainView (GridView Base) ] [ Html.text "Go to base" ]
                            , (grid model.pod model)
                            ]

                        Base ->
                            [ (grid model.base model)
                            , button [ onClick <| SetMainView (GridView Pod) ] [ Html.text "Go to pod" ]
                            ]
            in
                div
                    [ Html.Attributes.class "grid-container", helpers.class [ CenterContainer ] ]
                    contents

        _ ->
            div [] []


grid grid model =
    div [ helpers.class [ Container ] ]
        [ svg [ Svg.Attributes.width "100%", Svg.Attributes.height "100%" ]
            (List.concat
                (Grid.map slot grid)
            )
        , View.BuildDialog.view model
        ]


slot : Slot -> List (Svg Msg)
slot slot =
    case slot.entry of
        Just mod ->
            [ image (slotAttributes slot "module.svg") [] ]

        Nothing ->
            [ image (slotAttributes slot "grid_slot.svg") [] ]


slotAttributes : Slot -> String -> List (Svg.Attribute Msg)
slotAttributes slot image =
    let
        ( xp, yp ) =
            toPercentage slot.position
    in
        [ Svg.Attributes.x xp
        , Svg.Attributes.y yp
        , Svg.Attributes.xlinkHref ("/static/img/" ++ image)
        , onClick (ShowBuildDialog (Just slot.position))
        , Svg.Attributes.width "10%"
        , Svg.Attributes.height "10%"
        , Svg.Attributes.class "slot"
        ]


toPercentage : Point -> ( String, String )
toPercentage point =
    ( (toString <| point.x * 10) ++ "%", (toString <| point.y * 10) ++ "%" )


type Classes
    = Container
    | CenterContainer
    | PodFocused
    | BaseFocused


gridSize : number
gridSize =
    80


rules : Stylesheet
rules =
    (stylesheet << namespace ns)
        [ Css.class Container
            [ maxWidth (vw gridSize)
            , maxHeight (vw gridSize)
            , width (vh gridSize)
            , height (vh gridSize)
            , displayFlex
            , justifyContent center
            , alignItems center
            , padding (vw (gridSize / 50))
            ]
        , Css.class CenterContainer
            [ displayFlex
            , justifyContent center
            , alignItems center
            , width (pct 100)
            , height (pct 100)
            , flexDirection column
            ]
        ]


ns : String
ns =
    "grid"


helpers : Html.CssHelpers.Namespace String class id msg
helpers =
    Html.CssHelpers.withNamespace ns
