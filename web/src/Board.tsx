import React, { useState, useEffect, useCallback } from 'react'

import * as game from 'game'

import './Board.css'

import blackPawn from './img/black-pawn.svg'
import blackKnight from './img/black-knight.svg'
import blackRook from './img/black-rook.svg'
import blackBishop from './img/black-bishop.svg'
import blackQueen from './img/black-queen.svg'
import blackKing from './img/black-king.svg'

import whitePawn from './img/white-pawn.svg'
import whiteKnight from './img/white-knight.svg'
import whiteRook from './img/white-rook.svg'
import whiteBishop from './img/white-bishop.svg'
import whiteQueen from './img/white-queen.svg'
import whiteKing from './img/white-king.svg'

const toBackgroundId = (index: number) => {
  return Math.floor(index / 8) % 2 === 0
    ? index % 2 === 0 ? 'white' : 'black'
    : index % 2 !== 0 ? 'white' : 'black'
}

const renderToken = (piece: game.Piece) => {
  switch (piece) {
    default: return <></>
    case game.Piece.BlackPawn: return <img src={blackPawn} alt='p' />
    case game.Piece.BlackKnight: return <img src={blackKnight} alt='n' />
    case game.Piece.BlackRook: return <img src={blackRook} alt='r' />
    case game.Piece.BlackBishop: return <img src={blackBishop} alt='b' />
    case game.Piece.BlackQueen: return <img src={blackQueen} alt='q' />
    case game.Piece.BlackKing: return <img src={blackKing} alt='k' />
    case game.Piece.WhitePawn: return <img src={whitePawn} alt='P' />
    case game.Piece.WhiteKnight: return <img src={whiteKnight} alt='N' />
    case game.Piece.WhiteRook: return <img src={whiteRook} alt='R' />
    case game.Piece.WhiteBishop: return <img src={whiteBishop} alt='B' />
    case game.Piece.WhiteQueen: return <img src={whiteQueen} alt='Q' />
    case game.Piece.WhiteKing: return <img src={whiteKing} alt='K' />
  }
}

interface IProps {
  board: game.Board
  color: game.Color
}

const Board = (props: IProps) => {
  const [board, setBoard] = useState<game.Piece[]>([])
  const [selected, setSelected] = useState<number>()
  const [highlighted, setHighlighted] = useState<number[]>([])

  useEffect(() => {
    setBoard(props.board.enumerate())
  }, [props.board])

  const select = useCallback((index: number) => {
    if (!board) return
    setHighlighted(props.board.legal_moves(index))
    setSelected(index)
  }, [board, props.board])

  const render = (code: number, index: number) =>
    <div
      className='CellBackground'
      key={index}
      onClick={() => select(index)}
      id={toBackgroundId(index)}
    >
      <div
        className='Cell'
        onClick={() => {
          if (highlighted.indexOf(index) < 0) return
        }}
        id={index === selected ? 'selected' : highlighted.findIndex(i => i === index) >= 0 ? 'highlighted' : ''}
      >
        {renderToken(code)}
      </div>
    </div>

  return (
    <>
      <div className='Board'>
        {board.map(render)}
      </div>
      <div>1</div>
      <div>2</div>
      <div>3</div>
      <div>4</div>
      <div>5</div>
      <div>6</div>
      <div>7</div>
      <div>8</div>
      <div>a b c d e f g h</div>
    </>
  )
}

export default Board;

