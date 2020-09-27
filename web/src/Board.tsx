import React, { useState, useCallback } from 'react'

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

/* enum Token { */
/*   NONE, */
/*   BLACK_PAWN, */
/*   BLACK_KNIGHT, */
/*   BLACK_ROOK, */
/*   BLACK_BISHOP, */
/*   BLACK_QUEEN, */
/*   BLACK_KING, */
/*   WHITE_PAWN, */
/*   WHITE_KNIGHT, */
/*   WHITE_ROOK, */
/*   WHITE_BISHOP, */
/*   WHITE_QUEEN, */
/*   WHITE_KING, */
/* } */

const toBackgroundId = (index: number) => {
  return Math.floor(index / 8) % 2 === 0
    ? index % 2 === 0 ? 'white' : 'black'
    : index % 2 !== 0 ? 'white' : 'black'
}

const renderToken = (cell: game.Cell) => {
  switch () {
    default:
    case Token.NONE: return <></>
    case Token.BLACK_PAWN:  return <img src={blackPawn} alt='bk' />
    case Token.BLACK_KNIGHT:  return <img src={blackKnight} alt='bn' />
    case Token.BLACK_ROOK:  return <img src={blackRook} alt='br' />
    case Token.BLACK_BISHOP: return <img src={blackBishop} alt='bb' />
    case Token.BLACK_QUEEN: return <img src={blackQueen} alt='bq' />
    case Token.BLACK_KING: return <img src={blackKing} alt='bk' />
    case Token.WHITE_PAWN:  return <img src={whitePawn} alt='wk' />
    case Token.WHITE_KNIGHT:  return <img src={whiteKnight} alt='wn' />
    case Token.WHITE_ROOK:  return <img src={whiteRook} alt='wr' />
    case Token.WHITE_BISHOP: return <img src={whiteBishop} alt='wb' />
    case Token.WHITE_QUEEN: return <img src={whiteQueen} alt='wq' />
    case Token.WHITE_KING: return <img src={whiteKing} alt='wk' />
  }
}

const Board = () => {
  const [board, setBoard] = useState(new game.Board())
  const [selected, setSelected] = useState<number>()
  const [attack, setAttack] = useState<number>()
  const [highlighted, setHighlighted] = useState<number[]>([])

  const select = (index: number) => {
    const highlightList = []

    game.possible_moves_js(index, board)

    let cell = index - 8
    while (cell >= 0) {
      highlightList.push(cell)
      cell -= 8
    }

    cell = index + 8
    while (cell < 64) {
      highlightList.push(cell)
      cell += 8
    }

    setHighlighted(highlightList)
    setSelected(index)
  }

  const render = useCallback((token: Token, index: number) =>
    <div
      className='CellBackground'
      key={index}
      onClick={() => select(index)}
      id={toBackgroundId(index)}
    >
      <div
        className='Cell'
        id={index === selected ? 'selected' : highlighted.findIndex(i => i === index) >= 0 ? 'highlighted' : ''}
      >
        {renderToken(token)}
      </div>
    </div>, [selected, highlighted])

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

