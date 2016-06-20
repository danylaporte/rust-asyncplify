use atom::Atom;

pub fn replace_until_change<T, F>(atom: &Atom<Box<T>>, new_value: T, f: F)
    where F: Fn(Box<T>, Box<T>) -> Box<T>
{
    let mut new_value = Box::new(new_value);

    loop {
        let value = atom.take();
        let result;

        if let Some(actual_value) = value {
            result = f(actual_value, new_value);
        } else {
            result = new_value;
        }

        if let Some(nv) = atom.swap(result) {
            new_value = nv;
        } else {
            break;
        }
    }
}