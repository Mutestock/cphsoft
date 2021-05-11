import express, { Application, Request, Response, NextFunction } from 'express';
import neo4j from 'neo4j-driver';

const app: Application = express();
const add = (a: number, b: number): number => a + b;

let driver = neo4j.driver(
    'bolt://localhost:20501',
    neo4j.auth.basic('neo4j', 'some_password')
)
let session = driver.session()
  
//session
//  .run('MERGE (alice:Person {name : $nameParam}) RETURN alice.name AS name', {
//    nameParam: 'Alice'
//  })
//  .subscribe({
//    onKeys: keys => {
//      console.log(keys)
//    },
//    onNext: record => {
//      console.log(record.get('name'))
//    },
//    onCompleted: () => {
//      session.close() // returns a Promise
//    },
//    onError: error => {
//      console.log(error)
//    }
//  })

app.get('/', (req: Request, res: Response, next: NextFunction) => {
    res.send(String(add(5,5)));
})

app.get('/graph', (req: Request, res: Response, next: NextFunction) => {
  session
    .run('MERGE (alice:Person {name : $nameParam}) RETURN alice.name AS name', {
      nameParam: 'Alice'
    })
    .subscribe({
      onKeys: keys => {
        console.log(keys)
      },
      onNext: record => {
        console.log(record.get('name'))
      },
      onCompleted: () => {
        session.close() // returns a Promise
      },
      onError: error => {
        console.log(error)
      }
    });
})

app.listen(7993, () => console.log("Server running"));