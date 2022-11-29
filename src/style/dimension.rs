//! Style definitions for representing lengths

use crate::geometry::{Point, Rect, Size};

/// A unit of linear measurement
///
/// This is commonly combined with [`Rect`], [`Point`](crate::geometry::Point) and [`Size<T>`].
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LengthPercentage {
    /// Points are abstract absolute units. Users of Taffy may define what they correspond
    /// to in their application (pixels, logical pixels, mm, etc) as they see fit.
    Points(f32),
    /// The dimension is stored in percentage relative to the parent item.
    Percent(f32),
}

/// A unit of linear measurement
///
/// This is commonly combined with [`Rect`], [`Point`](crate::geometry::Point) and [`Size<T>`].
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LengthPercentageAuto {
    /// Points are abstract absolute units. Users of Taffy may define what they correspond
    /// to in their application (pixels, logical pixels, mm, etc) as they see fit.
    Points(f32),
    /// The dimension is stored in percentage relative to the parent item.
    Percent(f32),
    /// The dimension should be automatically computed
    Auto,
}

/// A unit of linear measurement
///
/// This is commonly combined with [`Rect`], [`Point`](crate::geometry::Point) and [`Size<T>`].
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Dimension {
    /// Points are abstract absolute units. Users of Taffy may define what they correspond
    /// to in their application (pixels, logical pixels, mm, etc) as they see fit.
    Points(f32),
    /// The dimension is stored in percentage relative to the parent item.
    Percent(f32),
    /// The dimension should be automatically computed
    Auto,
}

impl Dimension {
    /// Is this value defined?
    pub(crate) fn is_defined(self) -> bool {
        matches!(self, Dimension::Points(_) | Dimension::Percent(_))
    }
}

/// Returns the zero value for that type
pub const fn zero<T: TaffyZero>() -> T {
    T::ZERO
}

/// Trait to abstract over zero values
pub trait TaffyZero {
    /// The zero value for type implementing TaffyZero
    const ZERO: Self;
}
impl TaffyZero for f32 {
    const ZERO: f32 = 0.0;
}
impl TaffyZero for LengthPercentage {
    const ZERO: LengthPercentage = LengthPercentage::Points(0.0);
}
impl TaffyZero for LengthPercentageAuto {
    const ZERO: LengthPercentageAuto = LengthPercentageAuto::Points(0.0);
}
impl TaffyZero for Dimension {
    const ZERO: Dimension = Dimension::Points(0.0);
}
impl<T: TaffyZero> TaffyZero for Option<T> {
    const ZERO: Option<T> = Some(T::ZERO);
}
impl<T: TaffyZero> TaffyZero for Point<T> {
    const ZERO: Point<T> = Point { x: T::ZERO, y: T::ZERO };
}
impl<T: TaffyZero> Point<T> {
    /// Returns a Point where both the x and y values are the zero value of the contained type
    /// (e.g. 0.0, Some(0.0), or Dimension::Points(0.0))
    pub const fn zero() -> Self {
        zero::<Self>()
    }
}
impl<T: TaffyZero> TaffyZero for Size<T> {
    const ZERO: Size<T> = Size { width: T::ZERO, height: T::ZERO };
}
impl<T: TaffyZero> Size<T> {
    /// Returns a Size where both the width and height values are the zero value of the contained type
    /// (e.g. 0.0, Some(0.0), or Dimension::Points(0.0))
    pub const fn zero() -> Self {
        zero::<Self>()
    }
}
impl<T: TaffyZero> TaffyZero for Rect<T> {
    const ZERO: Rect<T> = Rect { left: T::ZERO, right: T::ZERO, top: T::ZERO, bottom: T::ZERO };
}
impl<T: TaffyZero> Rect<T> {
    /// Returns a Size where the left, right, top, and bottom values are all the zero value of the contained type
    /// (e.g. 0.0, Some(0.0), or Dimension::Points(0.0))
    pub const fn zero() -> Self {
        zero::<Self>()
    }
}

/// Returns the auto value for that type
pub const fn auto<T: TaffyAuto>() -> T {
    T::AUTO
}

/// Trait to abstract over auto values
pub trait TaffyAuto {
    /// The auto value for type implementing TaffyZero
    const AUTO: Self;
}
impl TaffyAuto for LengthPercentageAuto {
    const AUTO: LengthPercentageAuto = LengthPercentageAuto::Auto;
}
impl TaffyAuto for Dimension {
    const AUTO: Dimension = Dimension::Auto;
}
impl<T: TaffyAuto> TaffyAuto for Option<T> {
    const AUTO: Option<T> = Some(T::AUTO);
}
impl<T: TaffyAuto> TaffyAuto for Point<T> {
    const AUTO: Point<T> = Point { x: T::AUTO, y: T::AUTO };
}
impl<T: TaffyAuto> Point<T> {
    /// Returns a Point where both the x and y values are the auto value of the contained type
    /// (e.g. Dimension::Auto or LengthPercentageAuto::Auto)
    pub const fn auto() -> Self {
        auto::<Self>()
    }
}
impl<T: TaffyAuto> TaffyAuto for Size<T> {
    const AUTO: Size<T> = Size { width: T::AUTO, height: T::AUTO };
}
impl<T: TaffyAuto> Size<T> {
    /// Returns a Size where both the width and height values are the auto value of the contained type
    /// (e.g. Dimension::Auto or LengthPercentageAuto::Auto)
    pub const fn auto() -> Self {
        auto::<Self>()
    }
}
impl<T: TaffyAuto> TaffyAuto for Rect<T> {
    const AUTO: Rect<T> = Rect { left: T::AUTO, right: T::AUTO, top: T::AUTO, bottom: T::AUTO };
}
impl<T: TaffyAuto> Rect<T> {
    /// Returns a Size where the left, right, top, and bottom values are all the auto value of the contained type
    /// (e.g. Dimension::Auto or LengthPercentageAuto::Auto)
    pub const fn auto() -> Self {
        auto::<Self>()
    }
}

/// Returns a value of the inferred type which represent a constant of points
pub fn points<Input: Into<f32> + Copy, T: FromPoints>(points: Input) -> T {
    T::from_points(points)
}

/// Trait to create constant points values from plain numbers
pub trait FromPoints {
    /// Converts into an Into<f32> into Self
    fn from_points<Input: Into<f32> + Copy>(points: Input) -> Self;
}
impl FromPoints for f32 {
    fn from_points<Input: Into<f32> + Copy>(points: Input) -> Self {
        points.into()
    }
}
impl FromPoints for Option<f32> {
    fn from_points<Input: Into<f32> + Copy>(points: Input) -> Self {
        Some(points.into())
    }
}
impl FromPoints for LengthPercentage {
    fn from_points<Input: Into<f32> + Copy>(points: Input) -> Self {
        LengthPercentage::Points(points.into())
    }
}
impl FromPoints for LengthPercentageAuto {
    fn from_points<Input: Into<f32> + Copy>(points: Input) -> Self {
        LengthPercentageAuto::Points(points.into())
    }
}
impl FromPoints for Dimension {
    fn from_points<Input: Into<f32> + Copy>(points: Input) -> Self {
        Dimension::Points(points.into())
    }
}
impl<T: FromPoints> FromPoints for Point<T> {
    fn from_points<Input: Into<f32> + Copy>(points: Input) -> Self {
        Point { x: T::from_points(points.into()), y: T::from_points(points.into()) }
    }
}
impl<T: FromPoints> Point<T> {
    /// Returns a Point where both the x and y values are the constant points value of the contained type
    /// (e.g. 2.1, Some(2.1), or Dimension::Points(2.1))
    pub fn points<Input: Into<f32> + Copy>(points_value: Input) -> Self {
        points::<Input, Self>(points_value)
    }
}
impl<T: FromPoints> FromPoints for Size<T> {
    fn from_points<Input: Into<f32> + Copy>(points: Input) -> Self {
        Size { width: T::from_points(points.into()), height: T::from_points(points.into()) }
    }
}
impl<T: FromPoints> Size<T> {
    /// Returns a Size where both the width and height values are the constant points value of the contained type
    /// (e.g. 2.1, Some(2.1), or Dimension::Points(2.1))
    pub fn points<Input: Into<f32> + Copy>(points_value: Input) -> Self {
        points::<Input, Self>(points_value)
    }
}
impl<T: FromPoints> FromPoints for Rect<T> {
    fn from_points<Input: Into<f32> + Copy>(points: Input) -> Self {
        Rect {
            left: T::from_points(points.into()),
            right: T::from_points(points.into()),
            top: T::from_points(points.into()),
            bottom: T::from_points(points.into()),
        }
    }
}
impl<T: FromPoints> Rect<T> {
    /// Returns a Rect where the left, right, top and bottom values are all constant points value of the contained type
    /// (e.g. 2.1, Some(2.1), or Dimension::Points(2.1))
    pub fn points<Input: Into<f32> + Copy>(points_value: Input) -> Self {
        points::<Input, Self>(points_value)
    }

    /// Create a new Rect with [`Dimension::Points`]
    #[must_use]
    pub fn from_points<Input: Into<f32> + Copy>(start: Input, end: Input, top: Input, bottom: Input) -> Self {
        Rect {
            left: T::from_points(start),
            right: T::from_points(end),
            top: T::from_points(top),
            bottom: T::from_points(bottom),
        }
    }
}

/// Returns a value of the inferred type which represent a constant of points
pub fn percent<Input: Into<f32> + Copy, T: FromPercent>(percent: Input) -> T {
    T::from_percent(percent)
}

/// Trait to create constant points values from plain numbers
pub trait FromPercent {
    /// Converts into an Into<f32> into Self
    fn from_percent<Input: Into<f32> + Copy>(points: Input) -> Self;
}
impl FromPercent for LengthPercentage {
    fn from_percent<Input: Into<f32> + Copy>(points: Input) -> Self {
        LengthPercentage::Percent(points.into())
    }
}
impl FromPercent for LengthPercentageAuto {
    fn from_percent<Input: Into<f32> + Copy>(points: Input) -> Self {
        LengthPercentageAuto::Percent(points.into())
    }
}
impl FromPercent for Dimension {
    fn from_percent<Input: Into<f32> + Copy>(points: Input) -> Self {
        Dimension::Percent(points.into())
    }
}
impl<T: FromPercent> FromPercent for Point<T> {
    fn from_percent<Input: Into<f32> + Copy>(points: Input) -> Self {
        Point { x: T::from_percent(points.into()), y: T::from_percent(points.into()) }
    }
}
impl<T: FromPercent> Point<T> {
    /// Returns a Point where both the x and y values are the constant points value of the contained type
    /// (e.g. 2.1, Some(2.1), or Dimension::Points(2.1))
    pub fn percent<Input: Into<f32> + Copy>(percent_value: Input) -> Self {
        percent::<Input, Self>(percent_value)
    }
}
impl<T: FromPercent> FromPercent for Size<T> {
    fn from_percent<Input: Into<f32> + Copy>(points: Input) -> Self {
        Size { width: T::from_percent(points.into()), height: T::from_percent(points.into()) }
    }
}
impl<T: FromPercent> Size<T> {
    /// Returns a Size where both the width and height values are the constant points value of the contained type
    /// (e.g. 2.1, Some(2.1), or Dimension::Points(2.1))
    pub fn percent<Input: Into<f32> + Copy>(percent_value: Input) -> Self {
        percent::<Input, Self>(percent_value)
    }
}
impl<T: FromPercent> FromPercent for Rect<T> {
    fn from_percent<Input: Into<f32> + Copy>(points: Input) -> Self {
        Rect {
            left: T::from_percent(points.into()),
            right: T::from_percent(points.into()),
            top: T::from_percent(points.into()),
            bottom: T::from_percent(points.into()),
        }
    }
}
impl<T: FromPercent> Rect<T> {
    /// Returns a Rect where the left, right, top and bottom values are all constant points value of the contained type
    /// (e.g. 2.1, Some(2.1), or Dimension::Points(2.1))
    pub fn percent<Input: Into<f32> + Copy>(percent_value: Input) -> Self {
        percent::<Input, Self>(percent_value)
    }

    /// Create a new Rect with [`Dimension::Percent`]
    #[must_use]
    pub fn from_percent<Input: Into<f32> + Copy>(start: Input, end: Input, top: Input, bottom: Input) -> Self {
        Rect {
            left: T::from_percent(start),
            right: T::from_percent(end),
            top: T::from_percent(top),
            bottom: T::from_percent(bottom),
        }
    }
}
