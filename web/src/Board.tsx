import React, { useState, useCallback } from 'react'

import './Board.css'

import whitePawn from './img/white-pawn.svg'
import whiteKnight from './img/white-knight.svg'
import whiteRook from './img/white-rook.svg'
import whiteBishop from './img/white-bishop.svg'
import whiteQueen from './img/white-queen.svg'
import whiteKing from './img/white-king.svg'

import blackPawn from './img/black-pawn.svg'
import blackKnight from './img/black-knight.svg'
import blackRook from './img/black-rook.svg'
import blackBishop from './img/black-bishop.svg'
import blackQueen from './img/black-queen.svg'
import blackKing from './img/black-king.svg'

const toBackgroundId = (index: number) => {
  return Math.floor(index / 8) % 2 === 0
    ? index % 2 === 0 ? 'white' : 'black'
    : index % 2 !== 0 ? 'white' : 'black'
}

const idToToken = (id: number) => {
  switch (id) {
    case 1: return <img src={whitePawn} alt='wp' />
    case 2: return <img src={whiteKnight} alt='wkn' />
    case 3: return <img src={whiteRook} alt='wr' />
    case 4: return <img src={whiteBishop} alt='wb' />
    case 5: return <img src={whiteQueen} alt='wq' />
    case 6: return <img src={whiteKing} alt='wk' />

    case 7:  return <img src={blackPawn} alt='bk' />
    case 8:  return <img src={blackKnight} alt='bkn' />
    case 9:  return <img src={blackRook} alt='br' />
    case 10: return <img src={blackBishop} alt='bb' />
    case 11: return <img src={blackQueen} alt='bq' />
    case 12: return <img src={blackKing} alt='bk' />
  }
  return <></>
}

const Board = () => {
  const [board, setBoard] = useState([
    9,  8, 10, 11, 12, 10,  8,  9,
    7,  7,  7,  7,  7,  7,  7,  7,
    0,  0,  0,  0,  0,  0,  0,  0,
    0,  0,  0,  0,  0,  0,  0,  0,
    0,  0,  0,  0,  0,  0,  0,  0,
    0,  0,  0,  0,  0,  0,  0,  0,
    1,  1,  1,  1,  1,  1,  1,  1,
    3,  2,  4,  5,  6,  4,  2,  3,
  ])
  const [selected, setSelected] = useState<number>()
  const [attack, setAttack] = useState<number>()
  const [highlighted, setHighlighted] = useState<number[]>([])

  const select = (index: number) => {
    const highlightList = []

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

  const cell = useCallback((_: number, index: number) =>
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
        {idToToken(board[index])}
      </div>
    </div>, [selected, highlighted])

  return (
    <>
      <div className='Board'>
        {Array.from(Array(64))
          .map(cell)
        }
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

