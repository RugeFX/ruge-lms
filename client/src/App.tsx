import { useEffect, useState } from 'react'

interface Data {
  name: string,
  number: number
}

function App() {
  const [data, setData] = useState<Data | null>(null)

  useEffect(() => {
    (async () => {
      const res = await fetch("http://localhost:3000/123")
      const data: Data = await res.json()

      console.log(data);
      setData(data)
    })()
  })

  return (
    <main className='w-full flex flex-col gap-4 mx-28'>
      {data !== null && (
        <>
          <h1 className='text-3xl font-bold'>{data.name}</h1>
          <h2 className='text-lg font-semibold'>{data.number}</h2>
        </>
      )}
    </main>
  )
}

export default App
