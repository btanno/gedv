use super::*;

#[cfg(windows)]
use windows::Win32::Foundation::WPARAM;

pub type ButtonState = KeyState;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Ex(u32),
}

impl MouseButton {
    const EX_LEN: u32 = 29;

    fn as_u32(&self) -> u32 {
        match self {
            Self::Left => 0x01,
            Self::Right => 0x01 << 1,
            Self::Middle => 0x01 << 2,
            Self::Ex(x) => 0x01 << (x + 3),
        }
    }
}

impl std::fmt::Display for MouseButton {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct MouseButtons(u32);

impl MouseButtons {
    #[inline]
    pub fn new() -> Self {
        Self(0)
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub fn contains(&self, button: MouseButton) -> bool {
        let button = button.as_u32();
        self.0 & button == button
    }

    pub fn to_vec(&self) -> Vec<MouseButton> {
        let mut v = vec![];
        if self.contains(MouseButton::Left) {
            v.push(MouseButton::Left);
        }
        if self.contains(MouseButton::Right) {
            v.push(MouseButton::Right);
        }
        if self.contains(MouseButton::Middle) {
            v.push(MouseButton::Middle);
        }
        for i in 0..MouseButton::EX_LEN {
            if self.contains(MouseButton::Ex(i)) {
                v.push(MouseButton::Ex(i));
            }
        }
        v
    }
}

impl Default for MouseButtons {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for MouseButtons {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = self.to_vec();
        write!(f, "{:?}", v)
    }
}

impl std::fmt::Display for MouseButtons {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::ops::BitOr for MouseButton {
    type Output = MouseButtons;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        MouseButtons(self.as_u32() | rhs.as_u32())
    }
}

impl std::ops::BitOr<MouseButtons> for MouseButton {
    type Output = MouseButtons;

    #[inline]
    fn bitor(self, rhs: MouseButtons) -> Self::Output {
        MouseButtons(self.as_u32() | rhs.0)
    }
}

impl std::ops::BitOr<MouseButton> for MouseButtons {
    type Output = MouseButtons;

    #[inline]
    fn bitor(self, rhs: MouseButton) -> Self::Output {
        MouseButtons(self.0 | rhs.as_u32())
    }
}

impl std::ops::BitOrAssign for MouseButtons {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl std::ops::BitOrAssign<MouseButton> for MouseButtons {
    #[inline]
    fn bitor_assign(&mut self, rhs: MouseButton) {
        self.0 |= rhs.as_u32();
    }
}

impl <const N: usize> From<[MouseButton; N]> for MouseButtons {
    #[inline]
    fn from(value: [MouseButton; N]) -> Self {
        value.iter().fold(MouseButtons::new(), |r, b| r | *b)
    }
}

impl <const N: usize> From<&[MouseButton; N]> for MouseButtons {
    #[inline]
    fn from(value: &[MouseButton; N]) -> Self {
        value.iter().fold(MouseButtons::new(), |r, b| r | *b)
    }
}

impl From<&[MouseButton]> for MouseButtons {
    #[inline]
    fn from(value: &[MouseButton]) -> Self {
        value.iter().fold(MouseButtons::new(), |r, b| r | *b)
    }
}

impl From<Vec<MouseButton>> for MouseButtons {
    #[inline]
    fn from(value: Vec<MouseButton>) -> Self {
        value.iter().fold(MouseButtons::new(), |r, b| r | *b)
    }
}

impl From<&Vec<MouseButton>> for MouseButtons {
    #[inline]
    fn from(value: &Vec<MouseButton>) -> Self {
        value.iter().fold(MouseButtons::new(), |r, b| r | *b)
    }
}

#[cfg(windows)]
impl From<WPARAM> for MouseButtons {
    fn from(value: WPARAM) -> Self {
        Self((value.0 & 0xffff) as u32)
    }
}

#[derive(Clone, Debug)]
pub struct MouseState {
    pub buttons: MouseButtons,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MouseWheelAxis {
    Vertical,
    Horizontal,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mouse_buttons_contains() {
        let btns = MouseButtons(MouseButton::Left.as_u32());
        assert!(btns.contains(MouseButton::Left));
        assert!(!btns.contains(MouseButton::Right));
    }

    #[test]
    fn mouse_buttons_bitor() {
        let btns = MouseButton::Left | MouseButton::Right;
        assert!(btns.contains(MouseButton::Left));
        assert!(btns.contains(MouseButton::Right));
        assert!(!btns.contains(MouseButton::Middle));
        assert!(!btns.contains(MouseButton::Ex(0)));
        let btns = btns | MouseButton::Middle;
        assert!(btns.contains(MouseButton::Left));
        assert!(btns.contains(MouseButton::Right));
        assert!(btns.contains(MouseButton::Middle));
        assert!(!btns.contains(MouseButton::Ex(0)));
    }

    #[test]
    fn array_to_mouse_buttons() {
        let a = [MouseButton::Left, MouseButton::Right];
        let btns = MouseButtons::from(a);
        assert!(btns.contains(MouseButton::Left));
        assert!(btns.contains(MouseButton::Right));
        assert!(!btns.contains(MouseButton::Middle));
        assert!(!btns.contains(MouseButton::Ex(0)));
        let btns = MouseButtons::from(a.as_slice());
        assert!(btns.contains(MouseButton::Left));
        assert!(btns.contains(MouseButton::Right));
        assert!(!btns.contains(MouseButton::Middle));
        assert!(!btns.contains(MouseButton::Ex(0)));
        let v = vec![MouseButton::Left, MouseButton::Right];
        let btns = MouseButtons::from(v);
        assert!(btns.contains(MouseButton::Left));
        assert!(btns.contains(MouseButton::Right));
        assert!(!btns.contains(MouseButton::Middle));
        assert!(!btns.contains(MouseButton::Ex(0)));
        let v = vec![MouseButton::Left, MouseButton::Right];
        let btns = MouseButtons::from(&v);
        assert!(btns.contains(MouseButton::Left));
        assert!(btns.contains(MouseButton::Right));
        assert!(!btns.contains(MouseButton::Middle));
        assert!(!btns.contains(MouseButton::Ex(0)));
    }
}
