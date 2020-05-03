using { MyEntity} from '../db/schema';
service MyService {
    entity MySEntity as projection on MyEntity;
}